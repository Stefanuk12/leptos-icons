use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" y = "3" x = "3" width = "18" rx = "2" ></ rect > < / > } } pub const LucideSquare : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round")] } ;