use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "21" y1 = "6" y2 = "6" x2 = "3" ></ line > < line x1 = "15" x2 = "3" y1 = "12" y2 = "12" ></ line > < line y1 = "18" x2 = "3" y2 = "18" x1 = "17" ></ line > < / > } } pub const LucideAlignLeft : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;