use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "22 17 13.5 8.5 8.5 13.5 2 7" ></ polyline > < polyline points = "16 17 22 17 22 11" ></ polyline > < / > } } pub const LUCIDE_TRENDING_DOWN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;