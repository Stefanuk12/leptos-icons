use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "10 15 15 20 20 15" ></ polyline > < path d = "M4 4h7a4 4 0 0 1 4 4v12" ></ path > < / > } } pub const LUCIDE_CORNER_RIGHT_DOWN : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;