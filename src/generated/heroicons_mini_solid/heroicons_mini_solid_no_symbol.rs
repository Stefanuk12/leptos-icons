use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path clip - rule = "evenodd" fill - rule = "evenodd" d = "m5.965 4.904 9.131 9.131a6.5 6.5 0 0 0-9.131-9.131Zm8.07 10.192L4.904 5.965a6.5 6.5 0 0 0 9.131 9.131ZM4.343 4.343a8 8 0 1 1 11.314 11.314A8 8 0 0 1 4.343 4.343Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_NO_SYMBOL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("viewBox" , "0 0 20 20") , ("fill" , "currentColor")] } ;