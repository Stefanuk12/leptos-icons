use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8.7 3A6 6 0 0 1 18 8a21.3 21.3 0 0 0 .6 5" ></ path > < path d = "M17 17H3s3-2 3-9a4.67 4.67 0 0 1 .3-1.7" ></ path > < path d = "M10.3 21a1.94 1.94 0 0 0 3.4 0" ></ path > < path d = "m2 2 20 20" ></ path > < / > } } pub const LUCIDE_BELL_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;