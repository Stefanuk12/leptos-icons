use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 2h6l3 7H6l3-7Z" ></ path > < path d = "M12 9v13" ></ path > < path d = "M9 22h6" ></ path > < / > } } pub const LucideLampFloor : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;