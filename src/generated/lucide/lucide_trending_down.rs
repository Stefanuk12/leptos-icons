use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "22 17 13.5 8.5 8.5 13.5 2 7" ></ polyline > < polyline points = "16 17 22 17 22 11" ></ polyline > < / > } } pub const LUCIDE_TRENDING_DOWN : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor")] } ;