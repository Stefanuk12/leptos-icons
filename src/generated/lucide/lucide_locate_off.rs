use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" x2 = "5" y2 = "12" y1 = "12" ></ line > < line y1 = "12" y2 = "12" x1 = "19" x2 = "22" ></ line > < line x2 = "12" y2 = "5" x1 = "12" y1 = "2" ></ line > < line x1 = "12" y1 = "19" y2 = "22" x2 = "12" ></ line > < path d = "M7.11 7.11C5.83 8.39 5 10.1 5 12c0 3.87 3.13 7 7 7 1.9 0 3.61-.83 4.89-2.11" ></ path > < path d = "M18.71 13.96c.19-.63.29-1.29.29-1.96 0-3.87-3.13-7-7-7-.67 0-1.33.1-1.96.29" ></ path > < line x2 = "22" y2 = "22" x1 = "2" y1 = "2" ></ line > < / > } } pub const LUCIDE_LOCATE_OFF : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;