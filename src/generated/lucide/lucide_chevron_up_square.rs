use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" y = "3" width = "18" x = "3" ></ rect > < path d = "m8 14 4-4 4 4" ></ path > < / > } } pub const LucideChevronUpSquare : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;