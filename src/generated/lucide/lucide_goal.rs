use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 13V2l8 4-8 4" ></ path > < path d = "M20.55 10.23A9 9 0 1 1 8 4.94" ></ path > < path d = "M8 10a5 5 0 1 0 8.9 2.02" ></ path > < / > } } pub const LucideGoal : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;