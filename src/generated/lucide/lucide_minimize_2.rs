use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 14 10 14 10 20" ></ polyline > < polyline points = "20 10 14 10 14 4" ></ polyline > < line x1 = "14" y2 = "3" x2 = "21" y1 = "10" ></ line > < line x1 = "3" x2 = "10" y1 = "21" y2 = "14" ></ line > < / > } } pub const LUCIDE_MINIMIZE_2 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;