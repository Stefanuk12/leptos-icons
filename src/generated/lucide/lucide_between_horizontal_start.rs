use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "7" rx = "1" y = "3" width = "13" x = "8" ></ rect > < path d = "m2 9 3 3-3 3" ></ path > < rect y = "14" x = "8" rx = "1" height = "7" width = "13" ></ rect > < / > } } pub const LucideBetweenHorizontalStart : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;