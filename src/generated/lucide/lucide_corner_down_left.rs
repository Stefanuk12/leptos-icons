use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "9 10 4 15 9 20" ></ polyline > < path d = "M20 4v7a4 4 0 0 1-4 4H4" ></ path > < / > } } pub const LUCIDE_CORNER_DOWN_LEFT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;