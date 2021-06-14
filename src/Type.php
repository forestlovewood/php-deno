<?php declare(strict_types=1);

namespace Deno;

final class Type
{
    public const JS = 'JS';
    public const JSX = 'JSX';
    public const TS = 'TS';
    public const TSX = 'TSX';

    /**
     * @param string $type
     * @return bool
     */
    public static function valid(string $type): bool
    {
        return in_array($type, [self::JS, self::JSX, self::TS, self::TSX]);
    }
}