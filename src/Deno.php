<?php declare(strict_types=1);

namespace Deno;

use FFI;
use RuntimeException;

final class Deno {
    private static ?FFI $ffi = null;

    public function __construct()
    {
        if (null === self::$ffi) {
            try {
                self::$ffi = FFI::scope('DENO');
            } catch (FFI\Exception $e) {
                if (false === str_starts_with($e->getMessage(), 'Failed loading scope')) {
                    throw $e;
                }

                $package_path = dirname(__DIR__);

                $header_path = $package_path . DIRECTORY_SEPARATOR . 'lib' . DIRECTORY_SEPARATOR . 'libdeno.h';

                if (false === file_exists($header_path)) {
                    throw new RuntimeException('Deno header file does not exist.');
                }

                $header = file_get_contents($header_path);

                if (1 !== preg_match('/^#define FFI_LIB "libdeno\.(dylib|so)"$/m', $header, $header_matches)) {
                    throw new RuntimeException('Unable to parse Deno header file.');
                }

                $library_path = $package_path . DIRECTORY_SEPARATOR . 'lib' . DIRECTORY_SEPARATOR . 'libdeno.' . $header_matches[1];

                $header = str_replace('#define FFI_LIB "libdeno.' . $header_matches[1] . '"', '#define FFI_LIB "' . $library_path . '"', $header);

                $temp_header = tmpfile();
                $temp_header_path = stream_get_meta_data($temp_header)['uri'];
                fwrite($temp_header, $header);
                self::$ffi = FFI::load($temp_header_path);
                fclose($temp_header);

                unset($e, $package_path, $header_path, $header, $header_matches, $library_path, $temp_header, $temp_header_path);
            }
        }
    }

    private function mapTypeToEnum(string $type): FFI\CData {
        $cast = self::$ffi->cast('Type', match ($type) {
            Type::JS => self::$ffi->JS,
            Type::JSX => self::$ffi->JSX,
            Type::TS => self::$ffi->TS,
            Type::TSX => self::$ffi->TSX,
        });

        return FFI::addr($cast);
    }

    /**
     * @param string $source
     * @param string $type
     * @param int $timeout
     * @return string
     */
    public function execute(string $source, string $type = Type::TS, int $timeout = 0): string {
        if (!Type::valid($type)) throw new \InvalidArgumentException('Invalid type supplied for execute()');

        $long = self::$ffi->new('long');
        $long->cdata = $timeout;
        return self::$ffi->execute($source, $this->mapTypeToEnum($type), FFI::addr($long));
    }

    /**
     * @param string $path
     * @param string $type
     * @param int $timeout
     * @return string
     */
    public function execute_file(string $path, string $type = Type::TS, int $timeout = 0): string {
        if (!Type::valid($type)) throw new \InvalidArgumentException('Invalid type supplied for execute_file()');

        $long = self::$ffi->new('long');
        $long->cdata = $timeout;
        return self::$ffi->execute_file($path, $this->mapTypeToEnum($type), FFI::addr($long));
    }
}