use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "10 9 15 4 20 9" ></ polyline > < path d = "M4 20h7a4 4 0 0 0 4-4V4" ></ path > < / > } } pub const LUCIDE_CORNER_RIGHT_UP : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("height" , "24")] } ;