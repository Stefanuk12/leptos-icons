use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line y2 = "13" y1 = "19" x1 = "13" x2 = "19" ></ line > < line x2 = "20" y1 = "16" x1 = "16" y2 = "20" ></ line > < line x2 = "21" y2 = "19" x1 = "19" y1 = "21" ></ line > < polyline points = "14.5 6.5 18 3 21 3 21 6 17.5 9.5" ></ polyline > < line x1 = "5" y2 = "18" x2 = "9" y1 = "14" ></ line > < line y1 = "17" x2 = "4" x1 = "7" y2 = "20" ></ line > < line y1 = "19" x1 = "3" x2 = "5" y2 = "21" ></ line > < / > } } pub const LUCIDE_SWORDS : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;