use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 17 10 11 4 5" ></ polyline > < line y2 = "19" x2 = "20" y1 = "19" x1 = "12" ></ line > < / > } } pub const LUCIDE_TERMINAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;