use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "9" x2 = "19" y1 = "9" x1 = "5" ></ line > < line x1 = "5" y1 = "15" y2 = "15" x2 = "19" ></ line > < line y2 = "19" x2 = "5" x1 = "19" y1 = "5" ></ line > < / > } } pub const LUCIDE_EQUAL_NOT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;