use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line x1 = "13" x2 = "19" y1 = "19" y2 = "13" ></ line > < line x1 = "16" x2 = "20" y2 = "20" y1 = "16" ></ line > < line x1 = "19" y2 = "19" y1 = "21" x2 = "21" ></ line > < polyline points = "14.5 6.5 18 3 21 3 21 6 17.5 9.5" ></ polyline > < line y1 = "14" x1 = "5" x2 = "9" y2 = "18" ></ line > < line x1 = "7" x2 = "4" y2 = "20" y1 = "17" ></ line > < line x1 = "3" y1 = "19" x2 = "5" y2 = "21" ></ line > < / > } } pub const LUCIDE_SWORDS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;