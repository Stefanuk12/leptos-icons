use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 4h1a3 3 0 0 1 3 3 3 3 0 0 1 3-3h1" ></ path > < path d = "M13 20h-1a3 3 0 0 1-3-3 3 3 0 0 1-3 3H5" ></ path > < path d = "M5 16H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1" ></ path > < path d = "M13 8h7a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-7" ></ path > < path d = "M9 7v10" ></ path > < / > } } pub const LUCIDE_TEXT_CURSOR_INPUT : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;