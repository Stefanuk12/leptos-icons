use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2" ></ polygon > < line y2 = "15.5" x1 = "12" y1 = "22" x2 = "12" ></ line > < polyline points = "22 8.5 12 15.5 2 8.5" ></ polyline > < polyline points = "2 15.5 12 8.5 22 15.5" ></ polyline > < line x1 = "12" y1 = "2" x2 = "12" y2 = "8.5" ></ line > < / > } } pub const LUCIDE_CODEPEN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;