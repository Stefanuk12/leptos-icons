use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2" ></ polygon > < line x1 = "12" y2 = "15.5" y1 = "22" x2 = "12" ></ line > < polyline points = "22 8.5 12 15.5 2 8.5" ></ polyline > < polyline points = "2 15.5 12 8.5 22 15.5" ></ polyline > < line x1 = "12" y2 = "8.5" y1 = "2" x2 = "12" ></ line > < / > } } pub const LUCIDE_CODEPEN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;