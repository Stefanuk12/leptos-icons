use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" x2 = "21" y1 = "6" y2 = "6" ></ line > < line x1 = "3" y1 = "12" y2 = "12" x2 = "21" ></ line > < line x1 = "3" x2 = "21" y1 = "18" y2 = "18" ></ line > < / > } } pub const LucideAlignJustify : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;