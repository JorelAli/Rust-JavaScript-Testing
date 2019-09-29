import Browser
import Html exposing (Html, Attribute, div, input, text, button, h1, h2, header, br, h3)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)
import Regex
import Sha256
import MD5
import SHA1

-- MAIN

main = Browser.sandbox { init = init, update = update, view = view }

-- MODEL

type alias Model = { content : String }

init : Model
init = { content = "" }

-- UPDATE

type Msg
  = Change String


update : Msg -> Model -> Model
update msg model =
  case msg of
    Change newContent ->
      { model | content = newContent }

type Hash = MD5 | SHA256 | SHA1

hash : Hash -> String -> String
hash hashType input = 
  case hashType of
    MD5 -> input |> MD5.hex |> String.toUpper

    SHA256 -> input |> Sha256.sha256 |> String.toUpper

    SHA1 -> input |> SHA1.fromString |> SHA1.toHex |> String.toUpper

-- VIEW

view : Model -> Html Msg
view model =
  div []
    [ header [] 
      [ 
        h1 [] [ text "Desktop Hasher" ]
      , h2 [] [ text "Easily hash strings" ]
      ]
    , br [] []
    , input [ class "form-control", placeholder "Enter string here", value model.content, onInput Change ] []
    , br [] []
    , h3 [] [ text "MD5 hash" ]
    , input [ class "form-control", value (hash MD5 model.content), readonly True ] []
    , br [] []
    , h3 [] [ text "SHA-1 hash" ]
    , input [ class "form-control", value (hash SHA1 model.content), readonly True ] []
    , br [] [] 
    , h3 [] [ text "SHA-256 hash" ]
    , input [ class "form-control", value (hash SHA256 model.content), readonly True ] []
    ]
