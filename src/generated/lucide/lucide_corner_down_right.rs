use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "15 10 20 15 15 20" ></ polyline > < path d = "M4 4v7a4 4 0 0 0 4 4h12" ></ path > < / > } } pub const LUCIDE_CORNER_DOWN_RIGHT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none")] } ;