use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line y2 = "13" x2 = "19" y1 = "19" x1 = "13" ></ line > < line y2 = "20" x1 = "16" y1 = "16" x2 = "20" ></ line > < line x2 = "21" y1 = "21" y2 = "19" x1 = "19" ></ line > < / > } } pub const LUCIDE_SWORD : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2")] } ;