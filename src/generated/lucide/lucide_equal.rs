use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "9" x2 = "19" x1 = "5" y2 = "9" ></ line > < line x1 = "5" y2 = "15" y1 = "15" x2 = "19" ></ line > < / > } } pub const LUCIDE_EQUAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;