use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "22 7 13.5 15.5 8.5 10.5 2 17" ></ polyline > < polyline points = "16 7 22 7 22 13" ></ polyline > < / > } } pub const LUCIDE_TRENDING_UP : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;