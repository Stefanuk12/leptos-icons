use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" y1 = "6" y2 = "6" x2 = "21" ></ line > < line y2 = "12" y1 = "12" x1 = "3" x2 = "21" ></ line > < line y2 = "18" y1 = "18" x1 = "3" x2 = "21" ></ line > < / > } } pub const LucideAlignJustify : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;