use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "6" ry = "6" y = "6" width = "20" x = "2" height = "12" ></ rect > < circle cy = "12" r = "2" cx = "16" ></ circle > < / > } } pub const LucideToggleRight : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none")] } ;