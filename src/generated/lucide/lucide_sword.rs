use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line x2 = "19" x1 = "13" y1 = "19" y2 = "13" ></ line > < line x2 = "20" y2 = "20" x1 = "16" y1 = "16" ></ line > < line y2 = "19" x1 = "19" x2 = "21" y1 = "21" ></ line > < / > } } pub const LUCIDE_SWORD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;