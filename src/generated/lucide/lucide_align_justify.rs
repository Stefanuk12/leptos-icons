use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" y1 = "6" y2 = "6" x2 = "21" ></ line > < line x2 = "21" x1 = "3" y1 = "12" y2 = "12" ></ line > < line y2 = "18" x2 = "21" x1 = "3" y1 = "18" ></ line > < / > } } pub const LucideAlignJustify : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;