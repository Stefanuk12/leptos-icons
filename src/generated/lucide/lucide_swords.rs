use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line y1 = "19" x2 = "19" x1 = "13" y2 = "13" ></ line > < line y2 = "20" x1 = "16" y1 = "16" x2 = "20" ></ line > < line x1 = "19" x2 = "21" y2 = "19" y1 = "21" ></ line > < polyline points = "14.5 6.5 18 3 21 3 21 6 17.5 9.5" ></ polyline > < line y2 = "18" y1 = "14" x1 = "5" x2 = "9" ></ line > < line y2 = "20" x2 = "4" y1 = "17" x1 = "7" ></ line > < line y1 = "19" y2 = "21" x2 = "5" x1 = "3" ></ line > < / > } } pub const LUCIDE_SWORDS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;