use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" width = "18" height = "18" x = "3" ></ rect > < circle cx = "12" cy = "12" r = "1" ></ circle > < / > } } pub const LucideDotSquare : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;