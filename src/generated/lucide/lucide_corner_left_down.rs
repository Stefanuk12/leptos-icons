use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14 15 9 20 4 15" ></ polyline > < path d = "M20 4h-7a4 4 0 0 0-4 4v12" ></ path > < / > } } pub const LUCIDE_CORNER_LEFT_DOWN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24")] } ;