use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" y1 = "6" x1 = "21" x2 = "3" ></ line > < line y1 = "12" y2 = "12" x2 = "9" x1 = "21" ></ line > < line x2 = "7" y2 = "18" y1 = "18" x1 = "21" ></ line > < / > } } pub const LucideAlignRight : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;