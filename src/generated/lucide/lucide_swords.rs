use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line y1 = "19" x2 = "19" y2 = "13" x1 = "13" ></ line > < line x1 = "16" x2 = "20" y1 = "16" y2 = "20" ></ line > < line y1 = "21" y2 = "19" x1 = "19" x2 = "21" ></ line > < polyline points = "14.5 6.5 18 3 21 3 21 6 17.5 9.5" ></ polyline > < line x2 = "9" x1 = "5" y1 = "14" y2 = "18" ></ line > < line y1 = "17" x1 = "7" x2 = "4" y2 = "20" ></ line > < line y2 = "21" y1 = "19" x2 = "5" x1 = "3" ></ line > < / > } } pub const LUCIDE_SWORDS : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none")] } ;