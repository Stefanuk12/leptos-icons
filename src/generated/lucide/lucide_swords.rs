use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line x2 = "19" y1 = "19" x1 = "13" y2 = "13" ></ line > < line y2 = "20" x2 = "20" y1 = "16" x1 = "16" ></ line > < line x2 = "21" y2 = "19" x1 = "19" y1 = "21" ></ line > < polyline points = "14.5 6.5 18 3 21 3 21 6 17.5 9.5" ></ polyline > < line y2 = "18" x2 = "9" y1 = "14" x1 = "5" ></ line > < line x1 = "7" y1 = "17" y2 = "20" x2 = "4" ></ line > < line y2 = "21" y1 = "19" x1 = "3" x2 = "5" ></ line > < / > } } pub const LUCIDE_SWORDS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;