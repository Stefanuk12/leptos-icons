use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 17H7l-4 4v-7" ></ path > < path d = "M14 17h1" ></ path > < path d = "M14 3h1" ></ path > < path d = "M19 3a2 2 0 0 1 2 2" ></ path > < path d = "M21 14v1a2 2 0 0 1-2 2" ></ path > < path d = "M21 9v1" ></ path > < path d = "M3 9v1" ></ path > < path d = "M5 3a2 2 0 0 0-2 2" ></ path > < path d = "M9 3h1" ></ path > < / > } } pub const LUCIDE_MESSAGE_SQUARE_DASHED : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;