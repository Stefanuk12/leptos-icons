use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "3 8 7 12 3 16" ></ polyline > < line y2 = "12" x1 = "21" y1 = "12" x2 = "11" ></ line > < line y1 = "6" y2 = "6" x1 = "21" x2 = "11" ></ line > < line x2 = "11" x1 = "21" y2 = "18" y1 = "18" ></ line > < / > } } pub const LUCIDE_INDENT_INCREASE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;