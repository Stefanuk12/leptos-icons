use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10" ></ path > < path d = "M9 11h6" ></ path > < path d = "M12 8v6" ></ path > < / > } } pub const LucideShieldPlus : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("height" , "24")] } ;