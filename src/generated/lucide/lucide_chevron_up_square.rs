use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" y = "3" rx = "2" x = "3" ></ rect > < path d = "m8 14 4-4 4 4" ></ path > < / > } } pub const LucideChevronUpSquare : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;