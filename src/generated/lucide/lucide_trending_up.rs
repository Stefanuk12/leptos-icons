use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "22 7 13.5 15.5 8.5 10.5 2 17" ></ polyline > < polyline points = "16 7 22 7 22 13" ></ polyline > < / > } } pub const LUCIDE_TRENDING_UP : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;