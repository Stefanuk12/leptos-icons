use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line x1 = "13" y1 = "19" x2 = "19" y2 = "13" ></ line > < line x1 = "16" y2 = "20" x2 = "20" y1 = "16" ></ line > < line x2 = "21" y1 = "21" x1 = "19" y2 = "19" ></ line > < polyline points = "14.5 6.5 18 3 21 3 21 6 17.5 9.5" ></ polyline > < line y1 = "14" y2 = "18" x2 = "9" x1 = "5" ></ line > < line y2 = "20" x2 = "4" x1 = "7" y1 = "17" ></ line > < line y1 = "19" y2 = "21" x2 = "5" x1 = "3" ></ line > < / > } } pub const LUCIDE_SWORDS : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;