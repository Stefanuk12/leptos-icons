use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 9v6" ></ path > < circle cx = "5" cy = "18" r = "3" ></ circle > < path d = "M12 3v18" ></ path > < circle cy = "6" r = "3" cx = "19" ></ circle > < path d = "M16 15.7A9 9 0 0 0 19 9" ></ path > < / > } } pub const LucideGitGraph : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;