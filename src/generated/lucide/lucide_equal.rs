use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "19" y2 = "9" y1 = "9" x1 = "5" ></ line > < line x1 = "5" y1 = "15" x2 = "19" y2 = "15" ></ line > < / > } } pub const LUCIDE_EQUAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24")] } ;