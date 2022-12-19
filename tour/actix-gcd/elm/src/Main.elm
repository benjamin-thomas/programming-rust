module Main exposing (..)

import Generated.Domain exposing (GcdParameters)
import Html exposing (text)


addParams : GcdParameters -> Int
addParams gcdParams =
    case gcdParams.other of
        Nothing ->
            0

        Just other ->
            gcdParams.n + gcdParams.m + other


main : Html.Html msg
main =
    text "Hello"
