use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line y1 = "19" x1 = "13" y2 = "13" x2 = "19" ></ line > < line y1 = "16" x2 = "20" y2 = "20" x1 = "16" ></ line > < line y1 = "21" y2 = "19" x2 = "21" x1 = "19" ></ line > < / > } } pub const LUCIDE_SWORD : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;