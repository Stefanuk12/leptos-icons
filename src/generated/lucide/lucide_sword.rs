use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line y1 = "19" y2 = "13" x2 = "19" x1 = "13" ></ line > < line x1 = "16" x2 = "20" y2 = "20" y1 = "16" ></ line > < line y1 = "21" y2 = "19" x2 = "21" x1 = "19" ></ line > < / > } } pub const LUCIDE_SWORD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;