use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 14 10 14 10 20" ></ polyline > < polyline points = "20 10 14 10 14 4" ></ polyline > < line y1 = "10" y2 = "3" x2 = "21" x1 = "14" ></ line > < line y2 = "14" x1 = "3" y1 = "21" x2 = "10" ></ line > < / > } } pub const LUCIDE_MINIMIZE_2 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;