use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "10 9 15 4 20 9" ></ polyline > < path d = "M4 20h7a4 4 0 0 0 4-4V4" ></ path > < / > } } pub const LUCIDE_CORNER_RIGHT_UP : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;