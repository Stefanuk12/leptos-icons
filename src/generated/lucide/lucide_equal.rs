use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "9" x1 = "5" x2 = "19" y2 = "9" ></ line > < line x1 = "5" y1 = "15" y2 = "15" x2 = "19" ></ line > < / > } } pub const LUCIDE_EQUAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;