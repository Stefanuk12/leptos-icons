use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 17 10 11 4 5" ></ polyline > < line x2 = "20" y1 = "19" x1 = "12" y2 = "19" ></ line > < / > } } pub const LUCIDE_TERMINAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;