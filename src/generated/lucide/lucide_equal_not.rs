use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "19" x1 = "5" y1 = "9" y2 = "9" ></ line > < line y1 = "15" x2 = "19" x1 = "5" y2 = "15" ></ line > < line x1 = "19" x2 = "5" y2 = "19" y1 = "5" ></ line > < / > } } pub const LUCIDE_EQUAL_NOT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;