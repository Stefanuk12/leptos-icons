use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "19" x1 = "5" y2 = "9" y1 = "9" ></ line > < line x1 = "5" x2 = "19" y1 = "15" y2 = "15" ></ line > < line x1 = "19" y2 = "19" y1 = "5" x2 = "5" ></ line > < / > } } pub const LucideEqualNot : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;