use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "3 8 7 12 3 16" ></ polyline > < line x2 = "11" y2 = "12" y1 = "12" x1 = "21" ></ line > < line y2 = "6" y1 = "6" x1 = "21" x2 = "11" ></ line > < line y2 = "18" x1 = "21" x2 = "11" y1 = "18" ></ line > < / > } } pub const LUCIDE_INDENT_INCREASE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;